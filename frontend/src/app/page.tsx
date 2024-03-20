import Link from "next/link";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-4">
      <div className="z-10 w-full items-center justify-between font-mono text-sm lg:flex">
        <Link href="/admin">
          <p className="text-blue-500">Admin</p>
        </Link>
        <Link href="/user">
          <p className="text-blue-500">User</p>
        </Link>
      </div>
    </main>
  );
}
