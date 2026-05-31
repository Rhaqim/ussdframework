"""
AI USSD webhook service — Python / FastAPI

Handles function calls from the ussdframework and answers user questions
using OpenAI. Responses are clipped to fit USSD character limits (~155 chars).

Start:
    pip install -r requirements.txt
    cp .env.example .env        # fill in OPENAI_API_KEY
    uvicorn main:app --host 0.0.0.0 --port 3000

The ussdframework POSTs the full USSDSession JSON to each function_url
defined in menu.json. The returned dict is stored at session.data[data_key]
and referenced in screen templates as {{ai_response.answer}}.
"""

import os
from collections import defaultdict

from dotenv import load_dotenv
from fastapi import FastAPI, HTTPException, Request
from openai import AsyncOpenAI

load_dotenv()

app = FastAPI(title="AI USSD Service")

# ── Configuration ─────────────────────────────────────────────────────────────

MODEL = os.getenv("OPENAI_MODEL", "gpt-4o-mini")
MAX_QUESTIONS = int(os.getenv("MAX_QUESTIONS_PER_SESSION", "3"))

# Conservative USSD character budget:
#   182 (gateway page limit) - 4 ("END ") - ~20 (screen template overhead) = ~158
# We use 155 to leave a small safety margin.
USSD_MAX_CHARS = int(os.getenv("USSD_MAX_RESPONSE_CHARS", "155"))

client = AsyncOpenAI(api_key=os.environ["OPENAI_API_KEY"])

# In-memory per-session question counter.
# Replace with Redis / a database for multi-instance or persistent rate-limiting.
_session_counts: dict[str, int] = defaultdict(int)

# ── System prompts ────────────────────────────────────────────────────────────

SYSTEM_PROMPTS: dict[str, str] = {
    "agriculture": (
        "You are a concise agricultural advisor for smallholder farmers in Africa. "
        "Answer questions about crops, soil, pests, livestock, and farming practices. "
        "Reply in plain text only — no markdown, no bullet points — in 150 characters or fewer."
    ),
    "finance": (
        "You are a concise personal finance advisor for people in Africa. "
        "Answer questions about budgeting, savings, mobile money, loans, and investments. "
        "Reply in plain text only — no markdown, no bullet points — in 150 characters or fewer."
    ),
    "economy": (
        "You are a concise economic analyst providing accessible insights for people in Africa. "
        "Answer questions about inflation, exchange rates, trade, employment, and policy. "
        "Reply in plain text only — no markdown, no bullet points — in 150 characters or fewer."
    ),
}

# ── Helpers ───────────────────────────────────────────────────────────────────


def _unwrap_ussd_data(entry: object) -> str:
    """
    Extract a plain Python string from a USSDData enum value serialised as JSON.

    The Rust framework serialises USSDData variants as:
        {"Str": "hello"} | {"Int": 42} | {"Float": 3.14} | {"Dict": {...}}
    A plain string or number is also accepted for forward-compatibility.
    """
    if entry is None:
        return ""
    if isinstance(entry, dict):
        val = entry.get("Str") or entry.get("Int") or entry.get("Float")
        return str(val) if val is not None else ""
    return str(entry)


def _get_question(session: dict) -> str:
    """Pull the user's typed question out of session.data['question']."""
    raw = session.get("data", {}).get("question")
    return _unwrap_ussd_data(raw).strip()


def _truncate(text: str) -> str:
    """Clip the AI answer so it fits on a single USSD page."""
    text = text.strip()
    if len(text) <= USSD_MAX_CHARS:
        return text
    # Trim to the last whole word within the limit, then append ellipsis
    clipped = text[: USSD_MAX_CHARS - 3].rstrip()
    last_space = clipped.rfind(" ")
    if last_space > USSD_MAX_CHARS // 2:
        clipped = clipped[:last_space]
    return clipped + "..."


# ── Routes ────────────────────────────────────────────────────────────────────


@app.get("/health")
def health() -> dict:
    return {"status": "ok"}


@app.post("/ai/ask/{topic}")
async def ask_ai(topic: str, request: Request) -> dict:
    """
    Webhook endpoint for each AI topic.

    Called by the ussdframework when a user reaches AgricultureFunctionScreen,
    FinanceFunctionScreen, or EconomyFunctionScreen.

    Expected request body (full USSDSession):
    {
        "session_id": "...",
        "msisdn": "+254700000000",
        "language": "en",
        "current_screen": "AgricultureFunctionScreen",
        "data": {
            "question": {"Str": "Best fertiliser for maize in dry season?"}
        }
    }

    The returned dict is stored at session.data["ai_response"].
    The screen template {{ai_response.answer}} renders the answer.
    """
    if topic not in SYSTEM_PROMPTS:
        raise HTTPException(
            status_code=404,
            detail=f"Unknown topic '{topic}'. Valid topics: {list(SYSTEM_PROMPTS)}",
        )

    session = await request.json()
    session_id: str = session.get("session_id", "unknown")

    # ── Per-session rate limit ────────────────────────────────────────────────
    count = _session_counts[session_id]
    if count >= MAX_QUESTIONS:
        return {
            "answer": (
                f"You have reached the {MAX_QUESTIONS}-question limit. "
                "Dial again to start a new session."
            ),
        }

    # ── Extract the user's question ───────────────────────────────────────────
    question = _get_question(session)
    if not question:
        return {"answer": "No question received. Please try again."}

    # ── Query OpenAI ──────────────────────────────────────────────────────────
    try:
        completion = await client.chat.completions.create(
            model=MODEL,
            messages=[
                {"role": "system", "content": SYSTEM_PROMPTS[topic]},
                {"role": "user", "content": question},
            ],
            # 80 tokens ≈ 60 words, more than enough for a 150-char target
            max_tokens=80,
            temperature=0.4,
        )
        raw_answer = completion.choices[0].message.content or "No response returned."
    except Exception as exc:
        # Log the full exception server-side; surface only a safe message to users
        print(f"[ERROR] OpenAI call failed (session={session_id}, topic={topic}): {exc}")
        raw_answer = "AI service unavailable. Please try again later."

    _session_counts[session_id] = count + 1

    return {
        "answer": _truncate(raw_answer),
        "topic": topic,
        "questions_used": count + 1,
        "questions_remaining": max(0, MAX_QUESTIONS - count - 1),
    }
