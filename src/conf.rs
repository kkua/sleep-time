pub const UI_SCHEME: &str = "scui:";

pub const SCITER_GLOBAL_SCRIPT: &str = r#"
import * as sctr from "@sciter";
import * as env from "@env";

// sctr.on("click", "a[href^=http]", function(evt, el) {
document.on('^click','a[href^="http"]', function(evt, el) {
    //console.log("target ->", el.attributes["target"]);
    env.launch(el.attributes["href"]); // will open the url in default browser
    return true; // consume the event
});

// VM.unhandledExceptionHandler = function(err) {
//     stdout.println(err);
//     view.msgbox {
//         type:#error,
//         title:"出现错误",
//         content:err,
//         buttons: [{id: #close, text: "关闭"}],
        
//         onClose: function() {
//             view.close();
//         }
//     }
//     view.close();
// }
"#;
