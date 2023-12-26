import Link from 'next/link'

export default function Home() {
  return (
    <main className="flex flex-col items-center justify-center min-h-screen py-2">
    <Link href="/user_form" />
    <Link href="/restaurant_form" />
    <Link href="/review_form" />
    <Link href="/show_content" />
    </main>
  )
}
