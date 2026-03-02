import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "MAESMA Dashboard",
  description:
    "Modular Agentic Earth System Modeling Arena — Monitoring & Control",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="antialiased">{children}</body>
    </html>
  );
}
