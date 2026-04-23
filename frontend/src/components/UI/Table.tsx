import React from "react";

import TableProps from "@/types/table.type";

const Table = <T,>({ data, columns, onPress }: TableProps<T>) => {
	return (
		<div className="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden">
			<div className="overflow-x-auto">
				<table className="w-full">
					<thead>
						<tr className="border-b border-slate-800">
							{columns.map(col => (
								<th
									key={col.key}
									scope="col"
									className="px-5 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider"
								>
									{col.title}
								</th>
							))}
						</tr>
					</thead>
					<tbody className="divide-y divide-slate-800">
						{data.length === 0 ? (
							<tr>
								<td
									colSpan={columns.length}
									className="px-5 py-8 text-center text-sm text-slate-500"
								>
									No data found.
								</td>
							</tr>
						) : (
							(data as any[]).map((item, index) => (
								<tr
									key={index}
									onClick={() =>
										onPress(item.name ?? item.screen_name ?? item.id)
									}
									className="hover:bg-slate-800/50 transition-colors cursor-pointer"
								>
									{columns.map(col => (
										<td
											key={col.key}
											className="px-5 py-3.5 text-sm text-slate-300 whitespace-nowrap"
										>
											{item[col.key] ?? "—"}
										</td>
									))}
								</tr>
							))
						)}
					</tbody>
				</table>
			</div>
		</div>
	);
};

export default Table;

