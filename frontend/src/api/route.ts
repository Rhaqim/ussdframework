import Service from "@/types/service.type";
import Screen, { MenuItem, RouterOption } from "@/types/screen.type";

const fetcher = {
	post: async (url: string, data: any) => {
		return fetch(url, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(data),
		}).then(response => response.json());
	},

	get: async (url: string) => {
		return fetch(url).then(response => response.json());
	},

	put: async (url: string, data: any) => {
		return fetch(url, {
			method: "PUT",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(data),
		}).then(response => response.json());
	},

	delete: async (url: string) => {
		return fetch(url, {
			method: "DELETE",
		}).then(response => response.json());
	},
};

type Entity = "services" | "screens" | "menu_items" | "router_options";

const apiService = <T>(entity: Entity) => ({
	create: (item: T) => {
		return fetcher.post(`/api/${entity}`, item);
	},

	update: (item: T) => {
		return fetcher.put(`/api/${entity}`, item);
	},

	getAll: () => {
		return fetcher.get(`/api/${entity}`);
	},

	get: (id: number) => {
		return fetcher.get(`/api/${entity}/${id}`);
	},

	delete: (id: number) => {
		return fetcher.delete(`/api/${entity}/${id}`);
	},

	getByQuery: (query: string) => {
		return fetcher.get(`/api/${entity}/multiple/${query}`);
	},
});

export const Services = apiService<Service>("services");
export const Screens = apiService<Screen>("screens");
export const MenuItems = apiService<MenuItem>("menu_items");
export const RouterOptions = apiService<RouterOption>("router_options");

export const uploadFile = async (file: File) => {
	const formData = new FormData();
	formData.append("file", file);
	return fetch("/api/upload", {
		method: "POST",
		headers: {
			"Content-Type": "multipart/form-data",
		},
		body: formData,
	}).then(response => response.json());
};

export const downloadFile = async (filename: string) => {
	return fetch(`/api/download/${filename}`).then(response => response.blob());
};
