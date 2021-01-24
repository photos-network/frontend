{@html svg}

<script>
import arrowLeft from '../../assets/arrow-left.svg';
import arrowRight from '../../assets/arrow-right.svg';
import book from '../../assets/book.svg';
import calendar from '../../assets/calendar.svg';
import chevronLeft from '../../assets/chevron-left.svg';
import chevronRight from '../../assets/chevron-right.svg';
import close from '../../assets/close.svg';
import folder from '../../assets/folder.svg';
import info from '../../assets/info.svg';
import list from '../../assets/list.svg';
import map from '../../assets/map.svg';
import meatballs from '../../assets/meatballs.svg';
import photo from '../../assets/photo.svg';
import play from '../../assets/play.svg';
import search from '../../assets/search.svg';
import shared from '../../assets/shared.svg';
import trash from '../../assets/trash.svg';
import upload from '../../assets/upload.svg';
import user from '../../assets/user.svg';

const icons = {
	arrowLeft,
	arrowRight,
	book,
	calendar,
	chevronLeft,
	chevronRight,
	close,
	folder,
	info,
	list,
	map,
	meatballs,
	photo,
	play,
	search,
	shared,
	trash,
	upload,
	user,
};

export let name = '';
$:svg = icons[name] || '';

</script>
