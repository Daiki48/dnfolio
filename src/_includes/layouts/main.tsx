import { globalStyle } from "../../_styles/global.ts";

export default (
  { title, url, children, meta }: Lume.Data,
  _helpers: Lume.Helpers
) => (
  <html lang={meta.lang}>
    <head>
      <meta charSet="UTF-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      {title ? <title>{`${title} - ${meta.name}`}</title> : <title>{meta.name}</title>}
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
			{
				url ? 
				<meta name="og:url" content={`${meta.ogUrl}${url}`} />
				:
				<meta name="og:url" content={meta.ogUrl} />
			}

			{title ? 
				<meta name="og:title" content={`${title} - ${meta.ogTitle}`} />
			:
				<meta name="og:title" content={meta.ogTitle} />
			}
			<meta name="og:site_name" content={meta.ogSiteName} />
			<meta name="og:image" content={meta.ogImage} />
			<meta name="og:description" content={meta.ogDescription} />
			<meta name="og:type" content={meta.ogType} />
    </head>
    <body className="bg-gray-100">{children}</body>
  </html>
);
