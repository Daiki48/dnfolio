import { globalStyle } from "../../_styles/global.ts";

export default ({ children, meta }: Lume.Data, _helpers: Lume.Helpers) => (
  <html lang={meta.lang}>
    <head>
      {/* Google Tag Manager */}
      <script
        dangerouslySetInnerHTML={{
          __html:
            `(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start': new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0], j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src= 'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f); })(window,document,'script','dataLayer','GTM-KVK67VZ3');`,
        }}
      >
      </script>
      {/* End Google Tag Manager */}

      <meta charSet="UTF-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <title>{meta.name}</title>
      <meta name="description" content={meta.description} />
      {meta.styles.map((style: string, index: number) => (
        <link key={index} rel="stylesheet" href={style} />
      ))}
      <style>{globalStyle}</style>
      <link rel="icon" href={meta.icon} />
      {meta.ogp.name.map((name: string, index: number) => (
        <meta key={index} name={name} content={meta.ogp.content[index]} />
      ))}
    </head>
    <body className="bg-gray-100">
      {/* Google Tag Manager (noscript) */}
      <noscript>
        <iframe
          src="https://www.googletagmanager.com/ns.html?id=GTM-KVK67VZ3"
          height="0"
          width="0"
          style={{ display: "none", visibility: "hidden" }}
        >
        </iframe>
      </noscript>
      {/* End Google Tag Manager (noscript) */}
      {children}
    </body>
  </html>
);
