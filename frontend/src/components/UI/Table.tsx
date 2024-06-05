import React from "react";

import TableProps from "@/types/table.type";

const Table = <T,>({ data, columns, onPress }: TableProps<T>) => {
	return (
		<div className="overflow-x-auto">
			<table className="min-w-full divide-y divide-gray-200">
				<thead className="bg-gray-50">
					<tr>
						{columns.map(column => (
							<th
								key={column.key}
								scope="col"
								className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
							>
								{column.title}
							</th>
						))}
					</tr>
				</thead>
				<tbody className="bg-white divide-y divide-gray-200">
					{data.map((item: any, index: number) => (
						<tr
							key={index}
							onClick={() => onPress(item.name || item.screen_name || item.id)}
							className="cursor-pointer hover:bg-gray-100 hover:bg-opacity-50"
						>
							{columns.map(column => (
								<td
									key={column.key}
									className="px-6 py-4 whitespace-nowrap text-black"
								>
									{item[column.key]}
								</td>
							))}
						</tr>
					))}
				</tbody>
			</table>
		</div>
	);
};

export default Table;
