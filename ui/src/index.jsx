/* @refresh reload */
import { render } from 'solid-js/web';
import 'spectre.css/src/spectre.scss';
import 'spectre.css/src/spectre-icons.scss';
import './index.css';
import App from './App';
import { onMount } from 'solid-js';


if (window.location.hostname == 'tauri.localhost') {
    document.addEventListener('contextmenu', e => {
        e.preventDefault();
        return false;
    }, { capture: true })

    document.addEventListener('selectstart', e => {
        e.preventDefault();
        return false;
    }, { capture: true })
}

render(() => <App />, document.getElementById('root'));

