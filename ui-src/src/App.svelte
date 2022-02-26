<script>
	import NumberInput from "./widgets/NumberInput.svelte";
	let window = Window.this;
	let settings = window.xcall("get_settings");
	let shutdownHour = 0;
	let shutdownMinute = 0;
	let autorun = settings.autorun;

	function setAutorun() {
		if (!window.xcall("autorun_when_boot", autorun)) {
			autorun = !autorun;
		}
	}

	function setShutdownTime() {
		window.xcall("set_shutdown", shutdownHour, shutdownMinute);
	}

	window.on("statechange", function () {
		settings = window.xcall("get_settings");
	});
</script>

<main>
	<input
		type="checkbox"
		id="autorun"
		on:click={setAutorun}
		bind:checked={autorun}
	/><label for="autorun">开机自启动</label>
	<div>下次关机时间： {settings.shutdownTime}</div>
	<table>
		<tbody>
			<tr>
				<td>设置关机时间:</td>
				<td>
					<div>
						<NumberInput
							min="0"
							max="23"
							label="时"
							bind:value={shutdownHour}
						/>
						<NumberInput
							min="0"
							max="59"
							label="分"
							bind:value={shutdownMinute}
						/>
					</div>
				</td>
				<td>
					<button on:click={setShutdownTime}>设定</button>
				</td>
			</tr>
		</tbody>
	</table>
</main>