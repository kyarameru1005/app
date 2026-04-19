"use client";

import { useState } from "react";

const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL ?? "http://localhost:8080";

type LoginStartResponse = {
  message: string;
  auth_url: string;
};

export default function LoginPage() {
  const [errorMessage, setErrorMessage] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleLogin = async () => {
    setIsSubmitting(true);
    setErrorMessage("");
    try {
      const response = await fetch(`${apiBaseUrl}/api/auth/google/login`, {
        method: "POST",
        credentials: "include"
      });
      if (!response.ok) {
        throw new Error("ログインに失敗しました");
      }

      const payload = (await response.json()) as LoginStartResponse;
      const callbackResponse = await fetch(`${apiBaseUrl}${payload.auth_url}`, {
        method: "GET",
        credentials: "include"
      });
      if (!callbackResponse.ok) {
        throw new Error("認証処理に失敗しました");
      }
      window.location.href = "/memos";
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : "通信に失敗しました");
      setIsSubmitting(false);
    }
  };

  return (
    <main className="page">
      <section className="card">
        <h1>ログイン</h1>
        <p>Googleアカウントでログインしてください</p>
        <button
          type="button"
          className="google-button"
          onClick={handleLogin}
          disabled={isSubmitting}
        >
          Googleでログイン
        </button>
        {errorMessage && <p className="error-text">{errorMessage}</p>}
      </section>
    </main>
  );
}
