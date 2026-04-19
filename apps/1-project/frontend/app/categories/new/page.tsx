"use client";

import { FormEvent, useState } from "react";

const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL ?? "http://localhost:8080";

export default function CategoryCreatePage() {
  const [categoryName, setCategoryName] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setErrorMessage("");
    setIsSubmitting(true);
    try {
      const response = await fetch(`${apiBaseUrl}/api/categories`, {
        method: "POST",
        credentials: "include",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({ category_name: categoryName })
      });
      if (!response.ok) {
        const payload = (await response.json()) as { message?: string };
        throw new Error(payload.message ?? "カテゴリの登録に失敗しました");
      }
      window.location.href = "/categories";
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : "通信に失敗しました");
      setIsSubmitting(false);
    }
  };

  return (
    <main className="page">
      <section className="card">
        <h1>カテゴリ作成</h1>
        <form onSubmit={handleSubmit} className="form-block">
          <input
            type="text"
            value={categoryName}
            onChange={(event) => setCategoryName(event.target.value)}
            placeholder="カテゴリ名"
            maxLength={100}
          />
          <div className="button-row">
            <button type="submit" className="google-button" disabled={isSubmitting}>
              登録
            </button>
            <a href="/categories" className="outline-button">
              キャンセル
            </a>
          </div>
        </form>
        {errorMessage && <p className="error-text">{errorMessage}</p>}
      </section>
    </main>
  );
}
