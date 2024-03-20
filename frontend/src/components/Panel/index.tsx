"use client";

import React, { useState } from "react";
import ModelList from "@/components/Model/ModelList";
import ModelCreate from "@/components/Model/ModelCreate";
import ModelDelete from "@/components/Model/ModelDelete";
import Screen from "@/types/screen.type";
import Model from "@/types/model.type";


const AdminPanel: React.FC = () => {
	const [models, setModels] = useState<Model[]>([]);
	const [isCreateScreenVisible, setCreateScreenVisible] =
		useState<boolean>(false);

	const handleCreateModel = (screen: Screen) => {
		// Logic to create a new model
		const newModel: Model = {
			id: models.length + 1,
			name: "name",
			// other properties
		};
		setModels([...models, newModel]);
		setCreateScreenVisible(false); // Close create screen after model creation
	};

	const handleDeleteModel = (id: number) => {
		// Logic to delete a model
		const updatedModels = models.filter(model => model.id !== id);
		setModels(updatedModels);
	};

	const handleShowCreateScreen = () => {
		setCreateScreenVisible(true);
	};

	return (
		<div>
			<div className="mb-4">
				<h1 className="text-2xl font-semibold mb-2">Screens List</h1>
				<button
					onClick={handleShowCreateScreen}
					className="bg-blue-500 text-white px-4 py-2 rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
				>
					Create New Screen
				</button>
			</div>
			<ModelList models={models} />
			{isCreateScreenVisible && (
				<ModelCreate
					onCreate={handleCreateModel}
					// onCancel={() => setCreateScreenVisible(false)}
				/>
			)}
			{models.map(model => (
				<ModelDelete
					key={model.id}
					onDelete={handleDeleteModel}
					modelId={model.id}
				/>
			))}
		</div>
	);
};

export default AdminPanel;
