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
	try {
		const html = await utils.Search(await Answ.Anime);
		console.log(html);
	} catch (e) {
		console.log(e);
	}
}
main();
