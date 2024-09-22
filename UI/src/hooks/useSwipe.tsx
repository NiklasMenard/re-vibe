import { useState } from 'react';

const useSwipe = (onSwipe: (direction: 'left' | 'right') => void) => {
  const [startX, setStartX] = useState(0);

  const handleTouchStart = (e: React.TouchEvent<HTMLDivElement>) => {
    setStartX(e.touches[0].clientX);
  };

  const handleTouchEnd = (e: React.TouchEvent<HTMLDivElement>) => {
    const endX = e.changedTouches[0].clientX;
    const diffX = endX - startX;

    if (diffX > 50) {
      onSwipe('right');
    } else if (diffX < -50) {
      onSwipe('left');
    }
  };

  const swipeProps = {
    onTouchStart: handleTouchStart,
    onTouchEnd: handleTouchEnd,
    swipeStyle: { display: 'flex', alignItems: 'center', justifyContent: 'center' },
  };

  return swipeProps;
};

export default useSwipe;
