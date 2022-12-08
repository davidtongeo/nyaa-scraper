const Cheerio = require("cheerio");
const fetch = require("node-fetch");
class NyaaApi {
	async fetchPage(link) {
		const http = await fetch(link);
		const htmlt = await http.text();
		const $ = Cheerio.load(htmlt);
		return $;
	}
	async Search(Query) {
		let Response = [];
		const $ = await this.fetchPage(`https://nyaa.si/?f=0&c=0_0&q=${Query}`);
		const table = $("div.table-responsive > table > tbody >"); //tr is lefting
		table.each((i, el) => {
			$(el).each((ind, element) => {
				Response.push({
					Type: $(element).find("td").find("a").attr().title.trim(),
					Name: $(element).find("td").find("a").text().trim(),
					Size: $(element).find("td").next().next().next().html(),
					Magnet: $(element)
						.find("td")
						.next()
						.next()
						.find("a")
						.next()
						.attr()
						.href.trim()
						.replace(/(\r\n|\n|\r)/gm, ""),
				});
			});
		});
		return Response;
	}
}

module.exports = NyaaApi;
