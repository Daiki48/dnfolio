import { RC } from "react";

export const layout = "layouts/home.tsx";

export default ({ meta }: Lume.Data, _helpers: Lume.Helpers): RC => (
  <div className="flex flex-col items-center justify-center min-h-screen">
		<div className="my-4">
			<h1 className="text-4xl font-bold mb-8 animate-bounce">{meta.name}</h1>
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
      className="mb-8 text-blue-500 hover:border-b-2 hover:border-blue-500 animate-pulse hover:animate-none hover:font-bold"
    >
      Go to about pages
    </a>
  </div>
);
