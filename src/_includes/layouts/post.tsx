export const title = "Dnfolio post";
export const layout = "layouts/main.tsx";

export default ({ title, children }: Lume.Data, _helpers: Lume.Helpers) => (
  <>
    <div className="flex flex-col items-center justify-start">
      <a
        href="/"
        className="flex justify-center w-full max-w-2xl no-underline text-slate-500 hover:text-slate-800"
      >
        ← Home
      </a>
      <header className="text-center text-sm sm:text-xl font-bold text-wrap">
        <h1 className="leading-loose">{title}</h1>
      </header>
    </div>

    <main className="flex justify-center items-center max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
      {children}
    </main>
  </>
);
