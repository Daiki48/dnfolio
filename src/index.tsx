import { RC } from "react";

export const layout = "layouts/home.tsx";

export default ({ meta, search }: Lume.Data, _helpers: Lume.Helpers): RC => (
  <div className="flex flex-col items-center justify-center">
    <h1 className="text-4xl font-bold animate-bounce mt-8">{meta.name}</h1>
    <a href="/about" className="flex justify-center">
      <img
        src="/assets/icon/icon.webp"
        alt="logo"
        width="100%"
        height="50px"
        className="rounded"
      />
    </a>
    <div className="flex flex-col items-center justify-start min-h-screen">
      <div className="flex flex-col items-center justify-start">
        <ul className="flex flex-col justify-center items-start list-none pl-0">
          {search.pages("type=post", "date=desc").map((page, index) => (
            <li key={index} className="flex justify-between max-w-xl">
              <a
                href={page.url}
                className="text-left no-underline font-bold text-slate-500 hover:text-slate-800"
              >
                {page.title}
              </a>
            </li>
          ))}
        </ul>
      </div>
    </div>
  </div>
);
