export interface TableColumn {
	key: string;
	title: string;
}

declare interface TableProps<T = any>{
	columns: TableColumn[];
	data: T[];
	onPress: (id: string | number) => void;
}

export default TableProps;