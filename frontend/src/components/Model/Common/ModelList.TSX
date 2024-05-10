// ModelList.tsx
import React from 'react';

interface Model {
  id: number;
  name: string;
  // other properties
}

interface ModelListProps {
  models: Model[];
}

const ModelList: React.FC<ModelListProps> = ({ models }) => {
  return (
    <div>
      <h1>Model List</h1>
      <ul>
        {models.map(model => (
          <li key={model.id}>{model.name}</li>
          // render other properties if needed
        ))}
      </ul>
    </div>
  );
};

export default ModelList;
