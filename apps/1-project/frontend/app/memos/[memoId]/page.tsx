"use client";

import { useEffect, useState } from "react";

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

export default function MemoDetailPage({ params }: { params: { memoId: string } }) {
  const [memo, setMemo] = useState<Memo | null>(null);
  const [errorMessage, setErrorMessage] = useState("");

  useEffect(() => {
    const load = async () => {
      try {
        const response = await fetch(`${apiBaseUrl}/api/memos/${params.memoId}`, {
          credentials: "include"
        });
        if (!response.ok) {
          const payload = (await response.json()) as { message?: string };
          throw new Error(payload.message ?? "メモの取得に失敗しました");
        }
        const payload = (await response.json()) as Memo;
        setMemo(payload);
      } catch (error) {
        setErrorMessage(error instanceof Error ? error.message : "メモの取得に失敗しました");
      }
    };
    void load();
  }, [params.memoId]);

  return (
    <main className="page">
      <section className="card">
        <h1>メモ詳細</h1>
        {errorMessage && <p className="error-text">{errorMessage}</p>}
        {memo && (
          <>
            <p>
              <strong>タイトル:</strong> {memo.title}
            </p>
            <p>
              <strong>カテゴリ:</strong> {memo.category_name}
            </p>
            <p>
              <strong>本文:</strong> {memo.content || "本文なし"}
            </p>
            <div className="button-row">
              <a href={`/memos/${memo.memo_id}/edit`} className="outline-button">
                編集
              </a>
              <a href={`/memos/${memo.memo_id}/delete`} className="outline-button">
                削除
              </a>
              <a href="/memos" className="outline-button">
                一覧へ戻る
              </a>
            </div>
          </>
        )}
      </section>
    </main>
  );
}
