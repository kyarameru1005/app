import type { Metadata } from "next";
import "./styles.css";

export const metadata: Metadata = {
  title: "Memo App",
  description: "Google login based memo app"
};

export default function RootLayout({
  children
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body>{children}</body>
    </html>
  );
}
