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

export default function CategoryEditPage({
  params
}: {
  params: { categoryId: string };
}) {
  const [categoryName, setCategoryName] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    const loadCategory = async () => {
      try {
        const response = await fetch(`${apiBaseUrl}/api/categories`, {
          credentials: "include"
        });
        if (!response.ok) {
          throw new Error("カテゴリ一覧の取得に失敗しました");
        }
        const payload = (await response.json()) as CategoryListResponse;
        const current = payload.categories.find(
          (category) => category.category_id === params.categoryId
        );
        if (!current) {
          throw new Error("対象のカテゴリが存在しません");
        }
        setCategoryName(current.category_name);
      } catch (error) {
        setErrorMessage(
          error instanceof Error ? error.message : "カテゴリの取得に失敗しました"
        );
      }
    };
    void loadCategory();
  }, [params.categoryId]);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setErrorMessage("");
    setIsSubmitting(true);
    try {
      const response = await fetch(
        `${apiBaseUrl}/api/categories/${params.categoryId}`,
        {
          method: "PUT",
          credentials: "include",
          headers: {
            "Content-Type": "application/json"
          },
          body: JSON.stringify({ category_name: categoryName })
        }
      );
      if (!response.ok) {
        const payload = (await response.json()) as { message?: string };
        throw new Error(payload.message ?? "カテゴリの更新に失敗しました");
      }
      window.location.href = "/categories";
    } catch (error) {
      setErrorMessage(
        error instanceof Error ? error.message : "カテゴリの更新に失敗しました"
      );
      setIsSubmitting(false);
    }
  };

  return (
    <main className="page">
      <section className="card">
        <h1>カテゴリ編集</h1>
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
              更新
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
