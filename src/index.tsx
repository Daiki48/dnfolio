import { RC } from "react";

export const layout = "layouts/home.tsx";

export default ({ meta }: Lume.Data, _helpers: Lume.Helpers): RC => (
  <div className="flex flex-col items-center justify-center min-h-screen">
    <div className="flex flex-col justify-center my-2">
      <h1 className="text-4xl font-bold animate-bounce">{meta.name}</h1>
      <img
        src="/assets/icon/icon.webp"
        alt="logo"
        height="100px"
        width="auto"
        className="rounded"
      />
    </div>
    <a
      href="/about"
      className="no-underline m-2 p-2 text-teal-600 hover:border-b-2 shadow-lg rounded"
    >
      About
    </a>
  </div>
);
