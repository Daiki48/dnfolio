export const layout = "layouts/main.tsx";

export default (
  { title, children }: Lume.Data,
  _helpers: Lume.Helpers,
) => (
  <>
    <header className="text-center text-4xl font-bold my-8 pt-6">
      {title}
    </header>
    <main className="flex justify-center items-start max-w-full mx-auto px-4 sm:px-6 lg:px-8">
      <div className="w-full max-w-2xl break-words">
        <div className="flex flex-col justify-center items-center">
          <img
            src="/assets/icon/icon.webp"
            alt="logo"
            height="auto"
            width="200px"
            className="rounded-xl"
          />
          <div className="flex justify-center items-center p-4">
            <a href="https://x.com/Daiki48engineer" target="_blank">
              <img
                src="/assets/sns/x-logo.svg"
                alt="x"
                height="30px"
                width="auto"
                className="cursor-pointer px-2"
              />
            </a>
            <a href="https://sizu.me/daiki48" target="_blank">
              <img
                src="/assets/sns/sizu-logo.svg"
                alt="sizu"
                height="30px"
                width="auto"
                className="cursor-pointer px-2"
              />
            </a>
            <a
              href="https://bsky.app/profile/daiki48.bsky.social"
              target="_blank"
            >
              <img
                src="/assets/sns/bluesky-logo.svg"
                alt="bluesky"
                height="30px"
                width="auto"
                className="cursor-pointer px-2"
              />
            </a>
            <a href="https://github.com/Daiki48" target="_blank">
              <img
                src="/assets/sns/github-logo.svg"
                alt="github"
                height="30px"
                width="auto"
                className="cursor-pointer px-2"
              />
            </a>
          </div>
        </div>
        {children}
      </div>
    </main>
  </>
);
