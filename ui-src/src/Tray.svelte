<script>
    let window = Window.this;
    let trayExit = false;

    document.on("closerequest", function (evt, el) {
        if (!trayExit) {
            window.state = Window.WINDOW_HIDDEN;
            evt.preventDefault();
        }
    });

    document.ready = function () {
        window.trayIcon({ image: window.icon, text: window.caption });
        window.on("trayiconclick", function (evt) {
            let buttons = evt.data.buttons;
            if (buttons == 1) {
                toggleMainWindow();
            } else if (buttons == 2) {
                var [sx, sy] = window.box("position", "client", "screen", true);
                var menu = document.$("menu#tray");
                var { screenX, screenY } = evt.data;
                window.isTopmost = true;
                menu.popupAt(screenX - sx, screenY - sy, 2);
                document.on("popupdismissing", "menu#tray", function (evt, li) {
                    window.isTopmost = false;
                });
            }
        });
    };

    function toggleMainWindow() {
        let state = window.state;
        switch (window.state) {
            case Window.WINDOW_MINIMIZED:
            case Window.WINDOW_HIDDEN:
                window.state = Window.WINDOW_SHOWN;
                window.focus = window.document;
                break;
            case Window.WINDOW_MAXIMIZED:
            case Window.WINDOW_SHOWN:
            case Window.WINDOW_FULL_SCREEN:
                window.state = Window.WINDOW_HIDDEN;
                break;
        }
    }

    function reveal() {
        window.state = Window.WINDOW_SHOWN;
    }
    function exit() {
        window.trayIcon("remove");
        trayExit = true;
        window.close();
    }
</script>

<menu class="popup" id="tray">
    <li on:click={reveal}>设置</li>
    <li on:click={exit}>退出</li>
</menu>
