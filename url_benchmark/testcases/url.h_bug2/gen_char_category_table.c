// generated the array 'char_cat' which is #included and used in url.c

#include <stdio.h>

#include "url_char_category.h"

unsigned char_cat[256];

// character categories according to RFC 3986:
const char* const alnum="0123456789"
	"abcdefghijklmnopqrstuvwxyz"
	"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const char* const unreserved = "-._~";
const char* const subdelim   = "!$&'()*+,;=";

void fill(unsigned value, const char* alnum, const char* special)
{
	for(;*alnum;++alnum)
	{
		char_cat[ (unsigned char)(*alnum) ] |= value;
	}

	for(;*special;++special)
	{
		char_cat[ (unsigned char)(*special) ] |= value;
	}
}


void print_table()
{
	printf(
		"// This file is generated by gen_char_category_table. DO NOT EDIT IT BY HAND!\n"
		"\n"
		"static const unsigned short char_cat[256] = {\n"
		"//   .0     .1     .2     .3     .4     .5     .6     .7     .8     .9     .A     .B     .C     .D     .E     .F\n");
	
	for(unsigned y=0; y<16; ++y)
	{
		putchar(' ');
		for(unsigned x=0; x<16; ++x)
		{
			const unsigned offset = y*16+x;
			printf(" 0x%03x%c", char_cat[offset], (offset==255 ? ' ' : ',') );
		}
		printf(" // %01X0 ... %01XF\n", y, y);
	}
	printf("};\n\n");
}


int main()
{
	fill( Scheme    , alnum, "+-.");
	
	fill( Unreserved, alnum, unreserved);
	
	fill( GenDelim  , alnum, ":/?#[]@");
	
	fill( SubDelim  , alnum, subdelim);
	
	fill( PCharSlash, alnum, ":@/%");  // part #1
	fill( PCharSlash, unreserved, subdelim); // part #2
	
	fill( HexDigit, "0123456789", "abcdef" "ABCDEF");
	
	fill( Query, alnum, "/?:@%"); // part #1
	fill( Query, unreserved, subdelim); // part #2
	
	fill( Userinfo, alnum, ":%"); // part #1
	fill( Userinfo, unreserved, subdelim); // part #2
	
	fill( IPv6Char, "0123456789", "abcdef" "ABCDEF" ":");

	print_table();
}