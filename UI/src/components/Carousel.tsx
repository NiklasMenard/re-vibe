import useSwipe from '@/hooks/useSwipe';
import React, { useState } from 'react';
import { Skeleton } from './Skeleton';

interface CarouselProps {
  loading: boolean;
  children: React.ReactNode[];
}

const LeftToRightOverlay = () => {
  return (
    <div
      className="absolute top-0 left-0 right-0 bottom-0 bg-gradient-to-r from-black/5 to-black/70"
      aria-hidden="true"
    ></div>
  );
};

const RightToLeftOverlay = () => {
  return (
    <div
      className="absolute top-0 left-0 right-0 bottom-0 bg-gradient-to-l from-black/10 to-black/70"
      aria-hidden="true"
    ></div>
  );
};

const generateOverlay = (currentIndex: number, index: number) => {
  if (index === currentIndex) {
    return null;
  }

  if (index < currentIndex) {
    return <LeftToRightOverlay />;
  }

  if (index > currentIndex) {
    return <RightToLeftOverlay />;
  }
};

interface CardWrapperProps {
  index?: number;
  currentIndex?: number;
  style?: React.CSSProperties;
  onTouchStart?: (e: React.TouchEvent<HTMLDivElement>) => void;
  onTouchEnd?: (e: React.TouchEvent<HTMLDivElement>) => void;
  className?: string;
  children: React.ReactNode;
}

const CardWrapper: React.FC<CardWrapperProps> = ({ children, className, ...props }) => {
  return (
    <div
      className={`absolute flex items-center justify-center transition-transform duration-300 ease-linear ${className}`}
      {...props}
    >
      {children}
    </div>
  );
};

const Carousel: React.FC<CarouselProps> = ({ loading, children }) => {
  const [currentIndex, setCurrentIndex] = useState<number>(2);

  const prevSlide = (): void => {
    setCurrentIndex((prevIndex) => (prevIndex === 0 ? 0 : prevIndex - 1));
  };

  const nextSlide = (): void => {
    setCurrentIndex((prevIndex) =>
      prevIndex === children.length - 1 ? children.length - 1 : prevIndex + 1
    );
  };

  const calculateZIndex = (index: number) => {
    if (index === currentIndex) {
      return children.length;
    }

    if (index < currentIndex) {
      return index;
    }

    if (index > currentIndex) {
      return children.length - 1 - index;
    }
  };

  const handleSwipe = (direction: 'left' | 'right') => {
    if (direction === 'right') {
      nextSlide();
    } else if (direction === 'left') {
      prevSlide();
    }
  };

  const { swipeStyle, onTouchEnd, onTouchStart } = useSwipe(handleSwipe);

  const animatePosition = (index: number) => {
    // Center card
    if (index === currentIndex) {
      return 'translate-x-0 scale-110 opacity-100';
    }

    // Cards to the left of current card
    if (index < currentIndex) {
      return 'translate-x-[65%] scale-75 '; // Move to left
    }

    // Cards to the right of current card
    if (index > currentIndex) {
      return '-translate-x-[65%] scale-75 '; // Move to right
    }
  };

  return (
    <div className="flex h-full flex-col px-14 overflow-hidden">
      <div className="relative w-full h-full flex items-center">
        <div className="flex-shrink-0">
          <button
            onClick={prevSlide}
            className="p-2 text-jet hover:text-tangelo text-3xl hidden sm:block"
          >
            ◀
          </button>
        </div>

        <div className="flex items-center justify-center flex-grow">
          {loading
            ? // Render skeletons if loading
              [...Array(4)].map((_, skeletonIndex) => (
                <CardWrapper className={`${animatePosition(skeletonIndex)}`}>
                  <Skeleton
                    key={skeletonIndex}
                    className="h-40 w-40 md:h-80 md:w-80 rounded-[1rem]"
                  />
                </CardWrapper>
              ))
            : // Render cards when not loading
              children.map((card, index) => (
                <CardWrapper
                  key={index}
                  index={index}
                  currentIndex={currentIndex}
                  style={{
                    zIndex: calculateZIndex(index),
                    ...swipeStyle,
                  }}
                  onTouchStart={onTouchStart}
                  onTouchEnd={onTouchEnd}
                  className={`${animatePosition(index)}`}
                >
                  {card}
                  {generateOverlay(currentIndex, index)}
                </CardWrapper>
              ))}
        </div>

        <div className="flex-shrink-0">
          <button
            onClick={nextSlide}
            className="p-2 text-jet hover:text-tangelo text-3xl hidden sm:block"
          >
            ▶
          </button>
        </div>
      </div>

      <div className="flex justify-around lg:hidden touch-hidden">
        <button onClick={prevSlide} className="p-2 text-jet hover:text-tangelo text-3xl">
          ◀
        </button>
        <button onClick={nextSlide} className="p-2 text-jet hover:text-tangelo text-3xl">
          ▶
        </button>
      </div>
    </div>
  );
};

export default Carousel;
