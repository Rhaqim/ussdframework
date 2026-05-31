"""
Minimal USSD webhook handler — Python / FastAPI

Run with:
    pip install fastapi uvicorn
    uvicorn main:app --host 0.0.0.0 --port 3000

The ussdframework will POST the full USSDSession JSON to the endpoints
registered as `function_url` in your menu services config.

Session payload shape:
{
  "session_id": "...",
  "msisdn": "+254700000000",
  "language": "en",
  "current_screen": "CheckBalance",
  "data": {
    "account_number": {"Str": "1234567"}
  }
}

Return any JSON — it is stored at session.data[data_key].
"""

from fastapi import FastAPI, Request

app = FastAPI()


def get_session_value(session: dict, key: str) -> str | None:
    """Extract a plain string from the USSDData enum wrapper."""
    entry = session.get("data", {}).get(key)
    if entry is None:
        return None
    # USSDData is serialised as {"Str": "..."} | {"Int": 1} | {"Dict": {...}} etc.
    if isinstance(entry, dict):
        return str(entry.get("Str") or entry.get("Int") or entry.get("Float") or "")
    return str(entry)


@app.post("/ussd/check_balance")
async def check_balance(request: Request):
    session = await request.json()
    msisdn = session.get("msisdn", "")
    account = get_session_value(session, "account_number") or "unknown"

    # TODO: replace with a real database / API call
    balance = "KES 1,234.56"

    return {"balance": balance, "account": account, "msisdn": msisdn}


@app.post("/ussd/buy_airtime")
async def buy_airtime(request: Request):
    session = await request.json()
    amount = get_session_value(session, "airtime_amount") or "0"

    # TODO: call your airtime provider API
    success = True

    if success:
        return {"status": "success", "message": f"Airtime of {amount} purchased"}
    else:
        return {"status": "error", "message": "Purchase failed. Please try again."}


@app.post("/ussd/send_money")
async def send_money(request: Request):
    session = await request.json()
    recipient = get_session_value(session, "recipient_number") or ""
    amount = get_session_value(session, "transfer_amount") or "0"

    # TODO: call your payments API
    return {
        "status": "success",
        "message": f"KES {amount} sent to {recipient}",
        "transaction_id": "TXN-00001",
    }
