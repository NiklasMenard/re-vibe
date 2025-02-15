import HeartLiked from '@/assets/svg/heart-liked.svg?react';
import HeartUnliked from '@/assets/svg/heart-unliked.svg?react';

export interface FavoriteIconProps<T> {
  item?: T;
  isLiked?: boolean;
  className?: string;
  likeItem?: (item: T) => void;
  unlikeItem?: (item: T) => void;
}

const FavoriteIcon = <T extends { product_id: number }>({
  item,
  isLiked,
  likeItem,
  className,
  unlikeItem,
}: FavoriteIconProps<T>) => {
  const handleLike = () => {
    if (item && likeItem) {
      likeItem(item);
    }
  };

  const handleUnlike = () => {
    if (item && unlikeItem) {
      unlikeItem(item);
    }
  };

  return (
    <>
      {isLiked ? (
        <HeartLiked className={`${className} cursor-pointer`} onClick={handleUnlike} />
      ) : (
        <HeartUnliked className={`${className} cursor-pointer`} onClick={handleLike} />
      )}
    </>
  );
};

export default FavoriteIcon;
