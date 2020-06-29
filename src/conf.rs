pub const UI_SCHEME: &str = "scui:";

pub const SCITER_GLOBAL_SCRIPT: &str = r#"
self.on("click", "a[href^=http]", function(evt) {
    var href = this.attributes["href"];
    Sciter.launch(href); // will open the url in default browser
    return true; // consume the event
});

VM.unhandledExceptionHandler = function(err) {
    stdout.println(err);
    view.msgbox {
        type:#error,
        title:"出现错误",
        content:err,
        buttons: [{id: #close, text: "关闭"}],
        
        onClose: function() {
            view.close();
        }
    }
    view.close();
}
"#;
