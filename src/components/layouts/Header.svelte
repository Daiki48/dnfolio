<svelte:head>
	<title>Dnfolio</title>
</svelte:head>
<div class="main">
	<ul>
		<li>
			<a href="/"><span class="material-symbols-outlined"> home </span></a>
		</li>
		<li><a href="/about"><span class="material-symbols-outlined"> person </span></a></li>
		<li><a href="/blog"><span class="material-symbols-outlined"> note </span></a></li>
		<li><a href="/books"><span class="material-symbols-outlined"> book_2 </span></a></li>
	</ul>
</div>

<style>
	ul {
		display: flex;
		padding: 0;
	}

	ul li {
		list-style: none;
		margin: 1em 2em;
		position: relative;
		overflow: hidden;
	}

	ul li a:visited {
		color: inherit;
	}

	li::after {
		content: '';
		position: absolute;
		bottom: 0;
		left: -100%;
		height: 2px;
		width: 100%;
		background-color: #000;
		transition: left 0.5s;
	}

	li:hover::after {
		left: 0;
	}

	.main {
		display: flex;
		justify-content: center;
	}

	.material-symbols-outlined {
		color: rgb(3, 4, 5) !important;
	}
</style>
