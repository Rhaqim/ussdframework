// ModelDelete.tsx
import React from 'react';

interface ModelDeleteProps {
  onDelete: (id: number) => void;
  modelId: number;
}

const ModelDelete: React.FC<ModelDeleteProps> = ({ onDelete, modelId }) => {
  const handleDelete = () => {
    onDelete(modelId);
  };

  return (
    <div>
      <button onClick={handleDelete}>Delete Model</button>
    </div>
  );
};

export default ModelDelete;
