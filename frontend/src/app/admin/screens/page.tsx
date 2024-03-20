"use client";

import React, { useState } from "react";

import ModelList from "@/components/Model/ModelList";
import AdminPanel from "@/components/Panel";
import Model from "@/types/model.type";

const ScreenHomePage = () => {
	const [models, setModels] = useState<Model[]>([]);
	return (
		<div className="p-2">
			<ModelList models={models} />
		</div>
	);
};

export default ScreenHomePage;
