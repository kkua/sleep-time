import App from './App.svelte';
import Tray from './Tray.svelte';

const app = new App({
	target: document.body,
	props: {
		name: 'world'
	}
});

const tray = new Tray({
	target: document.head,
});

export default app;