"use client";

import { useState } from "react";

const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL ?? "http://localhost:8080";

export default function MemoDeleteConfirmPage({
  params
}: {
  params: { memoId: string };
}) {
  const [errorMessage, setErrorMessage] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const onDelete = async () => {
    setErrorMessage("");
    setIsSubmitting(true);
    try {
      const response = await fetch(`${apiBaseUrl}/api/memos/${params.memoId}`, {
        method: "DELETE",
        credentials: "include"
      });
      if (!response.ok) {
        const payload = (await response.json()) as { message?: string };
        throw new Error(payload.message ?? "メモの削除に失敗しました");
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
        <h1>メモ削除確認</h1>
        <p>このメモを削除しますか？</p>
        <div className="button-row">
          <button
            type="button"
            className="google-button"
            onClick={onDelete}
            disabled={isSubmitting}
          >
            削除する
          </button>
          <a href={`/memos/${params.memoId}`} className="outline-button">
            キャンセル
          </a>
        </div>
        {errorMessage && <p className="error-text">{errorMessage}</p>}
      </section>
    </main>
  );
}
