const NyaaApi = require("./NyaaClass.js");
const inquirer = require("inquirer");
//main shit!
async function main() {
	const utils = new NyaaApi();
	const Answ = await inquirer.prompt({
		name: "Anime",
		message: "Anime Query",
		default: "",
	});
	const AnswForMode = await inquirer.prompt({
		name: "Mode",
		message: "Which mode (leave it blank if u want the default, otherwise just add any character)",
		default: "obj"
	})
	try {
		if(AnswForMode.Mode == "obj"){
			const html = await utils.Search(await Answ.Anime);
			console.log(html);
		}
		else{
			const html = await utils.Search(await Answ.Anime);
			html.forEach(element => {
				console.log(element.Magnet);
			});
		}
	} catch (e) {
		console.log(e);
	}
}
main();
