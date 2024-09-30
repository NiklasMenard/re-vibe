import HeartLiked from '@/assets/svg/heart-liked.svg?react';
import HeartUnliked from '@/assets/svg/heart-unliked.svg?react';

export interface FavoriteIconProps<T> {
  item?: T;
  isLiked?: boolean;
  likeItem?: (item: T) => void;
  unlikeItem?: (item: T) => void;
}

const FavoriteIcon = <T extends { product_id: number }>({
  item,
  isLiked,
  likeItem,
  unlikeItem,
}: FavoriteIconProps<T>) => {
  const handleLike = () => {
    if (item && likeItem) {
      likeItem(item); // Call likeItem only if item and likeItem are defined
    }
  };

  const handleUnlike = () => {
    if (item && unlikeItem) {
      unlikeItem(item); // Call unlikeItem only if item and unlikeItem are defined
    }
  };

  return (
    <>
      {isLiked ? (
        <HeartLiked className="cursor-pointer" onClick={handleUnlike} />
      ) : (
        <HeartUnliked className="cursor-pointer" onClick={handleLike} />
      )}
    </>
  );
};

export default FavoriteIcon;
