export const globalStyle = `
	h1 {
		line-height: 2rem;
	}

	h2 {
		margin-top: 40px;
		border-bottom: 1px solid rgba(181, 179, 179, 1);
		padding-bottom: 10px;
	}

	p {
		line-height: 1.8rem;
	}

	code {
		padding: 6px;
		margin: 2px 4px;
		background-color: #e4e7ed;
		color: #cf0635;
		border-radius: 4px;
		font-weight: bold;
	}

	img {
		max-width: 80%;
		height: auto;
	}

blockquote {
	position: relative;
		border-left: 6px solid rgba(227, 227, 227, 1);
		border-radius: 6px;
		padding: 8px;
		background-color: rgba(227, 227, 227, 0.4);
	}
	blockquote cite {
		display: block;
		font-size: 0.8rem;
		text-align: right;
		color: rgba(126, 128, 130, 1);
		padding-right: 10px;
	}

	.markdown-alert {
		padding: 8px;
		border-radius: 4px;
	}
	.markdown-alert-note {
		background-color: rgba(150, 210, 255, 0.1);
		border: 2px solid rgba(0, 77, 135, 0.4);
	}
	.markdown-alert-note .markdown-alert-title {
		color: rgba(35, 66, 89, 1);
		font-weight: bold;
	}
	.markdown-alert-important {
		background-color: rgba(241, 148, 255, 0.1);
		border: 2px solid rgba(129, 3, 148, 0.4);
	}
	.markdown-alert-important .markdown-alert-title {
		color: rgba(86, 47, 92, 1);
		font-weight: bold;
	}
	.markdown-alert-tip {
		background-color: rgba(191, 255, 192, 0.1);
		border: 2px solid rgba(1, 143, 4, 0.4);
	}
	.markdown-alert-tip .markdown-alert-title {
		color: rgba(45, 173, 48, 1);
		font-weight: bold;
	}
	.markdown-alert-warning {
		background-color: rgba(252, 255, 166, 0.1);
		border: 2px solid rgba(133, 92, 5, 0.4);
	}
	.markdown-alert-warning .markdown-alert-title {
		color: rgba(219, 150, 0, 1);
		font-weight: bold;
	}
	.markdown-alert-caution {
		background-color: rgba(245, 141, 127, 0.1);
		border: 2px solid rgba(161, 20, 2, 0.4);
	}
	.markdown-alert-caution .markdown-alert-title {
		color: rgba(242, 88, 68, 1);
		font-weight: bold;
	}
`;
