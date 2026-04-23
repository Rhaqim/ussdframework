import React from 'react';

import MenuItemForm from '@/components/Model/Screen/Menu/Form';

const CreateMenuItemPage = () => {
  return (
    <div className="max-w-lg mx-auto py-8">
      <h1 className="text-2xl font-semibold text-white mb-6">Create Menu Item</h1>
      <MenuItemForm />
    </div>
  );
};

export default CreateMenuItemPage;
