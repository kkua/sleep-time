import styles from './App.module.css';
import { createEffect, createSignal, onCleanup, onMount } from 'solid-js';
import TimeSection from './TimeSection';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { message } from '@tauri-apps/api/dialog';
import { appWindow } from '@tauri-apps/api/window';
function App() {
  let clockInterval;
  let unlisten;
  let getHour;
  let getMinute;
  let [settings, setSettings] = createSignal({
    autorun: false,
    shutdownTime: "加载中",
  });

  let [clock, setClock] = createSignal(
    new Date()
  );

  onMount(() => {
    getSettings();
    clockInterval = setInterval(() => {
      setClock(new Date());
    }, 1000);
    unlisten = listen('will-shutdown', (event) => {
      alertShutdown();
    });
  });


  onCleanup(() => {
    clearInterval(clockInterval);
    unlisten.then(f => f());
  })

  async function alertShutdown() {
    await appWindow.show();
    await appWindow.setFullscreen(true);
    await appWindow.setFocus();
    await message('该睡了，此计算机将于30秒后关机！！！', { title: '早点睡觉', type: 'warning' });
    await appWindow.setFullscreen(false);
    getSettings();
  }

  function getSettings() {
    createEffect(() => {
      invoke("get_settings").then((resp) => setSettings(resp));
    })
  }

  async function setShutdown() {
    await invoke("set_shutdown", { hour: getHour(), minute: getMinute() });
    getSettings();
  }

  async function toggleAutorun() {
    let enable = !settings().autorun;
    await invoke("toggle_autorun", { enable });
    getSettings();
  }

  return (
    <>
      <div>
        <label class="form-switch">
          <input type="checkbox" checked={settings().autorun} onclick={toggleAutorun} />
          <i class="form-icon"></i>开机自启动
        </label>
      </div>
      <div>下次关机时间：{settings().shutdownTime}
      </div>

      <div>
        <label>设置关机时间：</label>
        <TimeSection min={0} max={23} init={0} bind={(getter) => getHour = getter} label="时" />
        <TimeSection min={0} max={59} init={0} bind={(getter) => getMinute = getter} label="分" />
        <button class='btn' onclick={setShutdown}>设定</button>
      </div>
      <div class="text-large bg-warning text-bold text-justify">
        {clock().toLocaleTimeString()}
      </div>
    </>
  );
}

export default App;
