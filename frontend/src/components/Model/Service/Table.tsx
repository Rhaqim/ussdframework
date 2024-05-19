import React from "react";

import Service from "@/types/service.type";
import TableBase from "@/components/UI/Table";

const Table = ({ data }: { data: Service[] }) => {
	return (
		<TableBase
			columns={[
				{ key: "id", title: "ID" },
				{ key: "name", title: "Name" },
				{ key: "function_name", title: "Function Name" },
				{ key: "function_url", title: "Function URL" },
				{ key: "data_key", title: "Data Key" },
				{ key: "service_code", title: "Service Code" },
			]}
			data={data}
		/>
	);
};

export default Table;