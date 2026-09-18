"use client";

import { useEffect, useState } from "react";

export default function WalletPage() {
  const [status, setStatus] = useState<string>("");

  useEffect(() => {
    // Simple status display for the wallet
    setStatus("HomeRail Student Wallet Connected");
  }, []);

  return (
    <div className="p-6 min-h-screen bg-background">
      <h1 className="text-3xl font-bold mb-6">
        HomeRail Student Wallet
      </h1>

      <div className="mb-6">
        <p className="text-gray-600 mb-3">Status: {status}</p>
        <p className="text-sm text-gray-500">
          This frontend connects to the Soroban HomeRail StudentWallet contract
        </p>
        <ul className="list-disc list-inside space-y-1">
          <li>Deposit USDC/XLM for a student</li>
          <li>Release portion to verified school</li>
          <li>Release free portion for campus spending</li>
          <li>Refund unclaimed funds after deadline</li>
        </ul>
      </div>
    </div>
  );
}