import { useState } from 'react';

const useSwipe = (onSwipe: (direction: 'left' | 'right', index: number) => void) => {
  const [startX, setStartX] = useState(0);

  const handleTouchStart = (e: React.TouchEvent<HTMLDivElement>) => {
    setStartX(e.touches[0].clientX);
  };

  const handleTouchEnd = (e: React.TouchEvent<HTMLDivElement>, index: number) => {
    const endX = e.changedTouches[0].clientX;
    const diffX = endX - startX;

    if (diffX > 50) {
      onSwipe('right', index);
    } else if (diffX < -50) {
      onSwipe('left', index);
    }
  };

  const swipeProps = {
    onTouchStart: handleTouchStart,
    onTouchEnd: handleTouchEnd,
  };

  return swipeProps;
};

export default useSwipe;
