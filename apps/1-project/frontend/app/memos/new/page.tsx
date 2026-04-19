"use client";

import { FormEvent, useEffect, useState } from "react";

const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL ?? "http://localhost:8080";

type Category = {
  category_id: string;
  category_name: string;
};

type CategoryListResponse = {
  categories: Category[];
};

export default function MemoCreatePage() {
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [categoryId, setCategoryId] = useState("");
  const [categories, setCategories] = useState<Category[]>([]);
  const [errorMessage, setErrorMessage] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    const load = async () => {
      try {
        const response = await fetch(`${apiBaseUrl}/api/categories`, {
          credentials: "include"
        });
        if (!response.ok) {
          const payload = (await response.json()) as { message?: string };
          throw new Error(payload.message ?? "カテゴリ一覧の取得に失敗しました");
        }
        const payload = (await response.json()) as CategoryListResponse;
        setCategories(payload.categories);
      } catch (error) {
        setErrorMessage(error instanceof Error ? error.message : "通信に失敗しました");
      }
    };
    void load();
  }, []);

  const onSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setErrorMessage("");
    setIsSubmitting(true);

    try {
      const response = await fetch(`${apiBaseUrl}/api/memos`, {
        method: "POST",
        credentials: "include",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          title,
          content,
          category_id: categoryId || null
        })
      });
      if (!response.ok) {
        const payload = (await response.json()) as { message?: string };
        throw new Error(payload.message ?? "メモの登録に失敗しました");
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
        <h1>メモ新規作成</h1>
        <form onSubmit={onSubmit} className="form-block">
          <input
            type="text"
            value={title}
            onChange={(event) => setTitle(event.target.value)}
            placeholder="タイトル"
            maxLength={200}
          />
          <textarea
            value={content}
            onChange={(event) => setContent(event.target.value)}
            placeholder="本文"
            rows={6}
          />
          <select
            value={categoryId}
            onChange={(event) => setCategoryId(event.target.value)}
          >
            <option value="">未分類</option>
            {categories.map((category) => (
              <option key={category.category_id} value={category.category_id}>
                {category.category_name}
              </option>
            ))}
          </select>
          <div className="button-row">
            <button type="submit" className="google-button" disabled={isSubmitting}>
              登録
            </button>
            <a href="/memos" className="outline-button">
              キャンセル
            </a>
          </div>
        </form>
        {errorMessage && <p className="error-text">{errorMessage}</p>}
      </section>
    </main>
  );
}
