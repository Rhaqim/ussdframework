import React from "react";

import { MenuItem } from "@/types/screen.type";
import TableBase from "@/components/UI/Table";

const Table = ({ data }: { data: MenuItem[] }) => {
	return (
		<TableBase
			columns={[
				{ key: "id", title: "ID" },
        { key: "screen_name", title: "Screen" },
        { key: "name", title: "Name" },
        { key: "option", title: "Option" },
        { key: "display_name", title: "Display Name" },
        { key: "next_screen", title: "Next Screen" },
			]}
			data={data}
			onPress={() => {console.log("Pressed")}}
		/>
	);
};

export default Table;