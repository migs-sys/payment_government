import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Proof of Payment for Government Services",
  description: "Proof of Payment for Government Services gives operators a shared settlement score trail, signed wallet actions, and a Soroban-backed release path that can be audited from dashboard to ledger.",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
