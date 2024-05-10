export interface TableColumn {
	key: string;
	title: string;
}

declare interface TableProps {
	columns: TableColumn[];
	data: any[];
}

export default TableProps;