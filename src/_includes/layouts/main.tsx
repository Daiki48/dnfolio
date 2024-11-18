import { globalStyle } from "../../_styles/global.ts";

export default ({ children, meta }: Lume.Data, _helpers: Lume.Helpers) => (
  <html lang={meta.lang}>
    <head>
      <meta charSet="UTF-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <title>{meta.name}</title>
      <meta name="description" content={meta.description} />
      {meta.styles.map((style: string, index: number) => (
        <link key={index} rel="stylesheet" href={style} />
      ))}
      <style>{globalStyle}</style>
      <link rel="icon" href={meta.icon} />
      <link
        rel="stylesheet"
        href="https://maxcdn.bootstrapcdn.com/font-awesome/4.7.0/css/font-awesome.min.css"
      />
      {meta.ogp.name.map((name: string, index: number) => (
        <meta key={index} name={name} content={meta.ogp.content[index]} />
      ))}
    </head>
    <body className="bg-gray-100">
      {children}
    </body>
  </html>
);
