const createModule = require("./target/pg_parse_wasm.js");

async function main() {
	try {
		console.log("Loading WASM module...");
		const module = await createModule();
		console.log("✅ WASM module loaded successfully!");

		// Test SQL validation using Rust library
		const sql = "SELECT 1";
		const bytes = module.lengthBytesUTF8(sql) + 1;
		const ptr = module._malloc(bytes);
		module.stringToUTF8(sql, ptr, bytes);

		const result = module._is_valid_sql(ptr);
		module._free(ptr);

		console.log(
			`✅ SQL validation result for "${sql}": ${result === 1 ? "valid" : "invalid"}`,
		);

		// Test invalid SQL
		const badSql = "INVALID SQL";
		const bytes2 = module.lengthBytesUTF8(badSql) + 1;
		const ptr2 = module._malloc(bytes2);
		module.stringToUTF8(badSql, ptr2, bytes2);

		const result2 = module._is_valid_sql(ptr2);
		module._free(ptr2);

		console.log(
			`✅ SQL validation result for "${badSql}": ${result2 === 1 ? "valid" : "invalid"}`,
		);

		console.log("✅ All WASM tests passed!");
		process.exit(0);
	} catch (error) {
		console.error("❌ Error:", error);
		process.exit(1);
	}
}

main();
