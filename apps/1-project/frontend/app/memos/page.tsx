"use client";

import { FormEvent, useEffect, useMemo, useState } from "react";

const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL ?? "http://localhost:8080";

type Memo = {
  memo_id: string;
  title: string;
  content: string;
  category_id?: string;
  category_name: string;
  created_at: string;
  updated_at: string;
};

type MemoListResponse = {
  memos: Memo[];
};

export default function MemoListPage() {
  const [keywordInput, setKeywordInput] = useState("");
  const [keyword, setKeyword] = useState("");
  const [memos, setMemos] = useState<Memo[]>([]);
  const [errorMessage, setErrorMessage] = useState("");

  const queryString = useMemo(() => {
    const params = new URLSearchParams();
    if (keyword.trim()) {
      params.set("keyword", keyword.trim());
    }
    const raw = params.toString();
    return raw ? `?${raw}` : "";
  }, [keyword]);

  useEffect(() => {
    const load = async () => {
      try {
        const response = await fetch(`${apiBaseUrl}/api/memos${queryString}`, {
          credentials: "include"
        });
        if (!response.ok) {
          throw new Error("メモ一覧の取得に失敗しました");
        }
        const payload = (await response.json()) as MemoListResponse;
        setMemos(payload.memos);
      } catch (error) {
        setErrorMessage(
          error instanceof Error ? error.message : "メモ一覧の取得に失敗しました"
        );
      }
    };
    void load();
  }, [queryString]);

  const onSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setKeyword(keywordInput);
  };

  return (
    <main className="page">
      <section className="card">
        <h1>メモ一覧</h1>
        <form onSubmit={onSubmit} className="form-block">
          <input
            type="text"
            value={keywordInput}
            onChange={(event) => setKeywordInput(event.target.value)}
            placeholder="タイトル・本文を検索"
          />
          <div className="button-row">
            <button type="submit" className="google-button">
              検索
            </button>
            <a href="/memos/new" className="link-button">
              新規作成
            </a>
            <a href="/categories" className="outline-button">
              カテゴリ管理
            </a>
          </div>
        </form>
        {errorMessage && <p className="error-text">{errorMessage}</p>}
        {!errorMessage && memos.length === 0 && <p>該当するメモがありません</p>}
        {!errorMessage && memos.length > 0 && (
          <ul className="list">
            {memos.map((memo) => (
              <li key={memo.memo_id} className="list-item">
                <strong>{memo.title}</strong> ({memo.category_name})
                <div className="button-row">
                  <a href={`/memos/${memo.memo_id}`} className="outline-button">
                    詳細
                  </a>
                  <a href={`/memos/${memo.memo_id}/edit`} className="outline-button">
                    編集
                  </a>
                  <a href={`/memos/${memo.memo_id}/delete`} className="outline-button">
                    削除
                  </a>
                </div>
              </li>
            ))}
          </ul>
        )}
      </section>
    </main>
  );
}
