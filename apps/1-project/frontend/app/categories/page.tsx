"use client";

import { useEffect, useState } from "react";

const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL ?? "http://localhost:8080";

type Category = {
  category_id: string;
  category_name: string;
};

type CategoryListResponse = {
  categories: Category[];
};

export default function CategoryListPage() {
  const [categories, setCategories] = useState<Category[]>([]);
  const [errorMessage, setErrorMessage] = useState("");

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

  return (
    <main className="page">
      <section className="card">
        <h1>カテゴリ一覧</h1>
        <div className="button-row">
          <a href="/categories/new" className="link-button">
            カテゴリ作成
          </a>
          <a href="/memos" className="outline-button">
            メモ一覧へ戻る
          </a>
        </div>

        {errorMessage && <p className="error-text">{errorMessage}</p>}
        {!errorMessage && categories.length === 0 && (
          <p>登録されているカテゴリがありません</p>
        )}
        {!errorMessage && categories.length > 0 && (
          <ul className="list">
            {categories.map((category) => (
              <li key={category.category_id} className="list-item">
                {category.category_name}{" "}
                <a
                  href={`/categories/${category.category_id}/edit`}
                  className="outline-button"
                >
                  編集
                </a>
              </li>
            ))}
          </ul>
        )}
      </section>
    </main>
  );
}
