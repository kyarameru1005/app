export default function MemoListPage() {
  return (
    <main className="page">
      <section className="card">
        <h1>メモ一覧</h1>
        <p>ログイン済みユーザー向けのメモ一覧画面です。</p>
        <a href="/categories" className="link-button">
          カテゴリ管理へ
        </a>
      </section>
    </main>
  );
}
