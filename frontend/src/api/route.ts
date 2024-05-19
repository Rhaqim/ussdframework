import Service from "@/types/service.type";
import Screen from "@/types/screen.type";

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

export const Services = {
	createService: (service: Service) => {
		return fetcher.post("/api/services", service);
	},

	updateService: (service: Service) => {
		return fetcher.put(`/api/services`, service);
	},

	getAllServices: () => {
		return fetcher.get(`/api/services`);
	},

	getService: (id: number) => {
		return fetcher.get(`/api/services/${id}`);
	},

	deleteService: (id: number) => {
		return fetcher.delete(`/api/services/${id}`);
	},

	getByQuery: (query: string) => {
		return fetcher.get(`/api/services/multiple/${query}`);
	},
};

export const Screens = {
	createScreen: (screen: Screen) => {
		return fetcher.post("/api/screens", screen);
	},

	updateScreen: (screen: Screen) => {
		return fetcher.put(`/api/screens`, screen);
	},

	getAllScreens: () => {
		return fetcher.get(`/api/screens`);
	},

	getScreen: (id: number) => {
		return fetcher.get(`/api/screens/${id}`);
	},

	deleteScreen: (id: number) => {
		return fetcher.delete(`/api/screens/${id}`);
	},

	getByQuery: (query: string) => {
		return fetcher.get(`/api/screens/multiple/${query}`);
	},
};

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
}

export const downloadFile = async (filename: string) => {
  return fetch(`/api/download/${filename}`).then(response => response.blob());
}